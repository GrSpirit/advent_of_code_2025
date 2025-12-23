// Advent of Code 2025 - Day 10: Factory (Part Two)
//
// We ignore the indicator light diagram for part two.
// For each machine:
// - There are m counters with target values b[0..m-1] (from {...})
// - There are k buttons; pressing button j increases each counter in its mask by 1
// - We may press each button an integer number of times x_j >= 0
//
// We need the minimum total presses:
//   minimize    sum_j x_j
//   subject to  A x = b,  x_j in Z_{\ge 0},  A_{i,j} in {0,1}
//
// In the real input, m <= 10 and (unique) buttons k <= 13, and the rational rank of A
// is usually close to k: the nullspace dimension (k - rank) is at most 3 for all lines.
// That makes an exact solution fast by:
// - Gaussian elimination over rationals to RREF
// - Enumerate only the <=3 free variables (bounded by b), compute pivots, keep best sum.

#include <algorithm>
#include <array>
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <string>
#include <string_view>
#include <unordered_set>
#include <utility>
#include <vector>

using std::array;
using std::int64_t;
using std::string;
using std::string_view;
using std::uint16_t;
using std::uint32_t;
using std::uint64_t;
using std::vector;

namespace {

static inline int popcount16(uint16_t x) {
  return __builtin_popcount(static_cast<unsigned>(x));
}

static inline int64_t igcd(int64_t a, int64_t b) {
  if (a < 0) a = -a;
  if (b < 0) b = -b;
  while (b != 0) {
    int64_t t = a % b;
    a = b;
    b = t;
  }
  return a;
}

static inline int64_t ilcm(int64_t a, int64_t b) {
  if (a == 0 || b == 0) return 0;
  int64_t g = igcd(a, b);
  __int128 v = static_cast<__int128>(a / g) * static_cast<__int128>(b);
  if (v > std::numeric_limits<int64_t>::max()) throw std::runtime_error("lcm overflow");
  return static_cast<int64_t>(v);
}

struct Rat {
  int64_t n = 0; // numerator
  int64_t d = 1; // denominator > 0

  Rat() = default;
  Rat(int64_t n_) : n(n_), d(1) {}
  Rat(int64_t n_, int64_t d_) : n(n_), d(d_) { normalize(); }

  void normalize() {
    if (d == 0) throw std::runtime_error("zero denominator");
    if (n == 0) {
      d = 1;
      return;
    }
    if (d < 0) {
      d = -d;
      n = -n;
    }
    int64_t g = igcd(n, d);
    n /= g;
    d /= g;
  }

  friend bool operator==(const Rat& a, const Rat& b) { return a.n == b.n && a.d == b.d; }
  friend bool operator!=(const Rat& a, const Rat& b) { return !(a == b); }

  friend Rat operator+(const Rat& a, const Rat& b) {
    int64_t g = igcd(a.d, b.d);
    __int128 ad = a.d / g;
    __int128 bd = b.d / g;
    __int128 num = static_cast<__int128>(a.n) * bd + static_cast<__int128>(b.n) * ad;
    __int128 den = ad * static_cast<__int128>(b.d);
    Rat r;
    r.n = static_cast<int64_t>(num);
    r.d = static_cast<int64_t>(den);
    r.normalize();
    return r;
  }

  friend Rat operator-(const Rat& a, const Rat& b) {
    return a + Rat(-b.n, b.d);
  }

  friend Rat operator*(const Rat& a, const Rat& b) {
    if (a.n == 0 || b.n == 0) return Rat(0);
    int64_t g1 = igcd(a.n, b.d);
    int64_t g2 = igcd(b.n, a.d);
    __int128 an = a.n / g1;
    __int128 bd = b.d / g1;
    __int128 bn = b.n / g2;
    __int128 ad = a.d / g2;
    __int128 num = an * bn;
    __int128 den = ad * bd;
    Rat r;
    r.n = static_cast<int64_t>(num);
    r.d = static_cast<int64_t>(den);
    r.normalize();
    return r;
  }

  friend Rat operator/(const Rat& a, const Rat& b) {
    if (b.n == 0) throw std::runtime_error("division by zero");
    Rat inv(b.d, b.n);
    inv.normalize();
    return a * inv;
  }

  Rat& operator+=(const Rat& o) { *this = *this + o; return *this; }
  Rat& operator-=(const Rat& o) { *this = *this - o; return *this; }
  Rat& operator*=(const Rat& o) { *this = *this * o; return *this; }
  Rat& operator/=(const Rat& o) { *this = *this / o; return *this; }

  friend bool is_zero(const Rat& a) { return a.n == 0; }
};

static inline void skip_spaces(string_view s, size_t& i) {
  while (i < s.size() && std::isspace(static_cast<unsigned char>(s[i]))) ++i;
}

static inline bool parse_int(string_view s, size_t& i, int& out) {
  skip_spaces(s, i);
  if (i >= s.size() || (!std::isdigit(static_cast<unsigned char>(s[i])) && s[i] != '-')) return false;
  bool neg = false;
  if (s[i] == '-') {
    neg = true;
    ++i;
  }
  int val = 0;
  bool any = false;
  while (i < s.size() && std::isdigit(static_cast<unsigned char>(s[i]))) {
    any = true;
    val = val * 10 + (s[i] - '0');
    ++i;
  }
  if (!any) return false;
  out = neg ? -val : val;
  return true;
}

static vector<int> parse_braced_list(string_view line) {
  size_t l = line.find('{');
  size_t r = line.find('}', l == string_view::npos ? 0 : l);
  if (l == string_view::npos || r == string_view::npos || r <= l) return {};
  string_view inside = line.substr(l + 1, r - (l + 1));
  vector<int> nums;
  size_t i = 0;
  while (i < inside.size()) {
    int x = 0;
    if (!parse_int(inside, i, x)) break;
    nums.push_back(x);
    skip_spaces(inside, i);
    if (i < inside.size() && inside[i] == ',') ++i;
  }
  return nums;
}

static vector<uint16_t> parse_button_masks(string_view line, int m) {
  vector<uint16_t> masks;
  size_t brace = line.find('{');
  size_t limit = (brace == string_view::npos) ? line.size() : brace;
  size_t i = 0;
  while (i < limit) {
    size_t l = line.find('(', i);
    if (l == string_view::npos || l >= limit) break;
    size_t r = line.find(')', l + 1);
    if (r == string_view::npos || r >= limit) break;
    string_view inside = line.substr(l + 1, r - (l + 1));
    uint16_t mask = 0;
    size_t j = 0;
    while (j < inside.size()) {
      int idx = 0;
      if (!parse_int(inside, j, idx)) break;
      if (idx >= 0 && idx < m) mask |= static_cast<uint16_t>(1u << idx);
      skip_spaces(inside, j);
      if (j < inside.size() && inside[j] == ',') ++j;
    }
    if (mask != 0) masks.push_back(mask);
    i = r + 1;
  }
  std::sort(masks.begin(), masks.end());
  masks.erase(std::unique(masks.begin(), masks.end()), masks.end());
  return masks;
}

struct ExprScaled {
  int64_t D = 1;             // denominator (positive)
  int64_t base = 0;          // numerator base, scaled by D
  vector<int64_t> coef;      // per free var, scaled by D
};

struct SolveMachine {
  int m = 0;
  int k = 0;
  vector<int> b;
  vector<uint16_t> masks;

  // RREF data:
  vector<vector<Rat>> A;  // m x k
  vector<Rat> rhs;        // m
  vector<int> pivot_col;  // size m, -1 if not pivot row
  vector<int> free_cols;  // columns that are free vars

  // Enumeration order:
  vector<int> free_order; // indices into free_cols (permuted)
  vector<int> ub;         // bounds for free vars in that order

  // Pivot expressions in terms of free vars in enumeration order.
  vector<ExprScaled> piv_exprs;

  int64_t best = (1LL << 60);
  vector<int> x; // assignment for free vars
  int64_t sum_free = 0;

  explicit SolveMachine(vector<int> b_, vector<uint16_t> masks_)
      : m(static_cast<int>(b_.size())), b(std::move(b_)), masks(std::move(masks_)) {
    k = static_cast<int>(masks.size());
    if (m == 0) throw std::runtime_error("empty machine");
    if (k == 0) throw std::runtime_error("no buttons");
    build_matrix();
    gauss_jordan_rref();
    build_parametrization();
    enumerate();
  }

  void build_matrix() {
    A.assign(m, vector<Rat>(k, Rat(0)));
    rhs.assign(m, Rat(0));
    for (int i = 0; i < m; ++i) rhs[i] = Rat(b[i]);
    for (int j = 0; j < k; ++j) {
      uint16_t mask = masks[j];
      for (int i = 0; i < m; ++i) {
        A[i][j] = Rat((mask & (1u << i)) ? 1 : 0);
      }
    }
  }

  void gauss_jordan_rref() {
    pivot_col.assign(m, -1);
    int row = 0;
    for (int col = 0; col < k && row < m; ++col) {
      int piv = -1;
      for (int r = row; r < m; ++r) {
        if (!is_zero(A[r][col])) { piv = r; break; }
      }
      if (piv == -1) continue;
      if (piv != row) {
        std::swap(A[piv], A[row]);
        std::swap(rhs[piv], rhs[row]);
      }

      Rat pv = A[row][col];
      for (int j = col; j < k; ++j) A[row][j] /= pv;
      rhs[row] /= pv;

      for (int r = 0; r < m; ++r) {
        if (r == row) continue;
        Rat f = A[r][col];
        if (is_zero(f)) continue;
        for (int j = col; j < k; ++j) A[r][j] -= f * A[row][j];
        rhs[r] -= f * rhs[row];
      }

      pivot_col[row] = col;
      ++row;
    }

    // Inconsistency check: 0 = nonzero.
    for (int r = 0; r < m; ++r) {
      bool all0 = true;
      for (int c = 0; c < k; ++c) {
        if (!is_zero(A[r][c])) { all0 = false; break; }
      }
      if (all0 && !is_zero(rhs[r])) throw std::runtime_error("inconsistent system");
    }
  }

  void build_parametrization() {
    std::unordered_set<int> pivcols;
    for (int r = 0; r < m; ++r) if (pivot_col[r] != -1) pivcols.insert(pivot_col[r]);
    free_cols.clear();
    for (int c = 0; c < k; ++c) if (!pivcols.contains(c)) free_cols.push_back(c);

    int d = static_cast<int>(free_cols.size());
    if (d > 3) throw std::runtime_error("unexpectedly large nullspace (d>3)");

    // Bounds for free vars from targets:
    // x_j <= min_{i in mask_j} b_i (and <= sumB)
    int sumB = 0;
    for (int v : b) sumB += v;
    vector<int> ub0(d, sumB);
    for (int fi = 0; fi < d; ++fi) {
      uint16_t mask = masks[free_cols[fi]];
      int bestb = sumB;
      for (int i = 0; i < m; ++i) {
        if (mask & (1u << i)) bestb = std::min(bestb, b[i]);
      }
      ub0[fi] = std::min(ub0[fi], bestb);
      if (ub0[fi] < 0) ub0[fi] = 0;
    }

    // Compute objective weights for ordering: w_f = 1 + sum_p coef_p,f (as rational -> long double).
    vector<long double> weight(d, 1.0L);
    for (int r = 0; r < m; ++r) {
      int pc = pivot_col[r];
      if (pc == -1) continue;
      for (int fi = 0; fi < d; ++fi) {
        int fc = free_cols[fi];
        Rat coef = Rat(0) - A[r][fc]; // x_pc = rhs - A[row][free]*x_free
        weight[fi] += static_cast<long double>(coef.n) / static_cast<long double>(coef.d);
      }
    }

    free_order.resize(d);
    std::iota(free_order.begin(), free_order.end(), 0);
    std::sort(free_order.begin(), free_order.end(), [&](int a, int b) {
      if (weight[a] != weight[b]) return weight[a] < weight[b];
      return ub0[a] < ub0[b];
    });

    ub.resize(d);
    for (int i = 0; i < d; ++i) ub[i] = ub0[free_order[i]];

    // Build scaled pivot expressions with coefficients in the enumeration order.
    piv_exprs.clear();
    for (int r = 0; r < m; ++r) {
      int pc = pivot_col[r];
      if (pc == -1) continue;

      Rat base = rhs[r];
      vector<Rat> coef_r(d, Rat(0));
      for (int oi = 0; oi < d; ++oi) {
        int fi = free_order[oi];
        int fc = free_cols[fi];
        coef_r[oi] = Rat(0) - A[r][fc];
      }

      int64_t D = base.d;
      for (int oi = 0; oi < d; ++oi) D = ilcm(D, coef_r[oi].d);
      if (D <= 0) throw std::runtime_error("bad denominator");

      ExprScaled e;
      e.D = D;
      e.coef.assign(d, 0);
      e.base = base.n * (D / base.d);
      for (int oi = 0; oi < d; ++oi) {
        e.coef[oi] = coef_r[oi].n * (D / coef_r[oi].d);
      }
      piv_exprs.push_back(std::move(e));
    }

    x.assign(d, 0);
  }

  bool can_still_be_nonneg(int pos) const {
    // For each pivot expr, check that it's possible to make numerator >= 0 with remaining vars.
    for (const auto& e : piv_exprs) {
      __int128 cur = e.base;
      for (int i = 0; i < pos; ++i) cur += static_cast<__int128>(e.coef[i]) * x[i];

      __int128 maxv = cur;
      for (int i = pos; i < static_cast<int>(x.size()); ++i) {
        if (e.coef[i] > 0) maxv += static_cast<__int128>(e.coef[i]) * ub[i];
      }
      if (maxv < 0) return false;
    }
    return true;
  }

  void dfs(int pos) {
    if (sum_free >= best) return; // pivots are nonnegative, so total can't beat best
    if (!can_still_be_nonneg(pos)) return;

    int d = static_cast<int>(x.size());
    if (pos == d) {
      int64_t total = sum_free;
      for (const auto& e : piv_exprs) {
        __int128 num = e.base;
        for (int i = 0; i < d; ++i) num += static_cast<__int128>(e.coef[i]) * x[i];
        if (num < 0) return;
        int64_t n64 = static_cast<int64_t>(num);
        if (n64 % e.D != 0) return;
        int64_t val = n64 / e.D;
        if (val < 0) return;
        total += val;
        if (total >= best) return;
      }
      best = std::min(best, total);
      return;
    }

    // Iterate value order:
    // If the objective weight is negative (often), trying larger first finds better solutions earlier.
    // We approximate by looking at how increasing x[pos] changes sum of pivot numerators.
    long double approx_w = 1.0L;
    for (const auto& e : piv_exprs) {
      approx_w += static_cast<long double>(e.coef[pos]) / static_cast<long double>(e.D);
    }
    int lo = 0, hi = ub[pos];
    if (approx_w < 0) {
      for (int v = hi; v >= lo; --v) {
        x[pos] = v;
        sum_free += v;
        dfs(pos + 1);
        sum_free -= v;
      }
    } else {
      for (int v = lo; v <= hi; ++v) {
        x[pos] = v;
        sum_free += v;
        dfs(pos + 1);
        sum_free -= v;
      }
    }
  }

  void enumerate() {
    dfs(0);
    if (best >= (1LL << 60)) throw std::runtime_error("no solution");
  }
};

static int64_t minimal_presses_for_machine(const vector<int>& b, const vector<uint16_t>& masks) {
  SolveMachine s(b, masks);
  return s.best;
}

} // namespace

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::ifstream fin("input.txt");
  if (!fin) {
    std::cerr << "Failed to open input.txt\n";
    return 1;
  }

  int64_t total = 0;
  string line;
  while (std::getline(fin, line)) {
    if (line.empty()) continue;
    vector<int> b = parse_braced_list(line);
    int m = static_cast<int>(b.size());
    vector<uint16_t> masks = parse_button_masks(line, m);
    total += minimal_presses_for_machine(b, masks);
  }

  std::cout << total << "\n";
  return 0;
}
