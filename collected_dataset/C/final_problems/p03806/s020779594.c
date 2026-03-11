/*

import "core"
import "std/algorithm"
import "procon"

const MAX_N := 41
const MAX_AB := 11
const MAX_C := 4001

n := read_int()
ma := read_int()
mb := read_int()

type Yakuhin {
  a Int
  b Int
  c Int
}

var yakuhins Array[40, Yakuhin]
var dp Array[MAX_AB, Array[MAX_AB, Int]]

fn min(a Int, b Int) Int {
  if (a < b) {
    a
  } else {
    b
  }
}

fn print_dp(a Int, b Int) {
  for i in range(1, a) {
    for j in range(1, b) {
      print(get(get(dp, i), j))
      print(" ")
    }
    println("")
  }
}

fn dp1() {
  for i in range(0, n-1) {
    yakuhin := get(yakuhins, i)
    if (get(get(dp, yakuhin.b), yakuhin.a) > yakuhin.c) {
      set(rget(dp, yakuhin.b), yakuhin.a, yakuhin.c)
    }
  }
  for cura in range(1, MAX_AB-1) {
    for curb in range(1, MAX_AB-1) {
      for preva in range(1, cura) {
        prevb := curb - preva
        ca := get(get(dp, prevb), preva)
        cb := get(get(dp, preva), prevb)
        if (get(get(dp, curb), cura) > ca + cb) {
          set(rget(dp, curb), cura, ca + cb)
        }
      }
    }
  }
}

fn main() {
  for i in range(0, n-1) {
    set(yakuhins, i, init(Yakuhin){read_int(); read_int(); read_int()})
  }
  for a in range(0, MAX_AB-1) {
    for b in range(0, MAX_AB-1) {
      set(rget(dp, b), a, int(INF))
    }
  }

  dp1()

  ans := INF
  for a in range(0, MAX_AB-1) {
    for b in range(0, MAX_AB-1) {
      if (a*mb == b*ma) {
        ans = min(ans, get(get(dp, b), a))
      }
    }
  }
  if (ans == INF) {
    println("-1")
  } else {
    println(ans)
  }
}

main()

*/
#define RELEASE

#include "stdbool.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"
#include "inttypes.h"
#include "stdio.h"
#include "math.h"


struct core_string_String {
char* p;
int64_t len;
};
struct core_vec_Vec_core_string_String {
struct core_string_String* data;
bool* isinit;
int64_t cap;
int64_t len;
};
struct Array1024_core_prelude_Char { char data[1024]; };
struct d_Yakuhin {
int64_t a;
int64_t b;
int64_t c;
};
struct Array40_d_Yakuhin { struct d_Yakuhin data[40]; };
struct Array11_core_prelude_Int { int64_t data[11]; };
struct Array11_core_array_Array_11_core_prelude_Int { struct Array11_core_prelude_Int data[11]; };

void* core_pointer_nullGG_();
char* core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_p, int64_t core_pointer_i);
char core_string_getGG_core_prelude_CString_core_prelude_Int(char* core_string_cs, int64_t core_string_i);
char core_string_charGG_core_prelude_Int(int64_t core_string_c);
char* core_pointer_allocGcore_prelude_CharG_core_prelude_Int(int64_t core_pointer_len);
void core_pointer_memcpyGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_dest, char* core_pointer_src, int64_t core_pointer_len);
struct core_string_String core_string_stringGG_core_prelude_CString(char* core_string_cs);
char* core_string_to_csGG_core_string_String(struct core_string_String core_string_s);
int64_t core_string_lengthGG_core_string_String(struct core_string_String core_string_s);
char* core_pointer_reallocGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_ptr, int64_t core_pointer_len);
void core_string_pushGG_core_string_String_core_prelude_CString(struct core_string_String* core_string_dest, char* core_string_src);
bool core_definitions_is_release_fnGG_();
bool core_definitions_is_debug_fnGG_();
int64_t procon_read_intGG_();
int64_t d_minGG_core_prelude_Int_core_prelude_Int(int64_t d_a, int64_t d_b);
void d_dp1GG_();
void d_mainGG_();


bool core_prelude_true;
bool core_prelude_false;
void* core_pointer_nullGG_() {
void* core_pointer_tmpid1 = ((void*)(0));
return core_pointer_tmpid1;
}
void* core_pointer_null;
float core_math_M_PI;
char* core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_p, int64_t core_pointer_i) {
char* core_pointer_tmpid35 = ((char*)((((int64_t)(core_pointer_p)) + core_pointer_i)));
return core_pointer_tmpid35;
}
char core_string_getGG_core_prelude_CString_core_prelude_Int(char* core_string_cs, int64_t core_string_i) {
char core_string_tmpid36 = (*(core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(((char*)(core_string_cs)), core_string_i)));
return core_string_tmpid36;
}
char core_string_charGG_core_prelude_Int(int64_t core_string_c) {
char core_string_tmpid39 = ((char)(core_string_c));
return core_string_tmpid39;
}
char* core_pointer_allocGcore_prelude_CharG_core_prelude_Int(int64_t core_pointer_len) {
char* core_pointer_tmpid38 = ((char*)(malloc((sizeof(char) * core_pointer_len))));
return core_pointer_tmpid38;
}
void core_pointer_memcpyGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_dest, char* core_pointer_src, int64_t core_pointer_len) {
memcpy(((void*)(core_pointer_dest)), ((void*)(core_pointer_src)), (sizeof(char) * core_pointer_len));
}
struct core_string_String core_string_stringGG_core_prelude_CString(char* core_string_cs) {
int64_t core_string_len = strlen(core_string_cs);
struct core_string_String core_string_s = (struct core_string_String){core_pointer_allocGcore_prelude_CharG_core_prelude_Int((core_string_len + 1)), core_string_len};
core_pointer_memcpyGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_string_s.p, ((char*)(core_string_cs)), core_string_len);
(*(core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_string_s.p, core_string_len))) = core_string_charGG_core_prelude_Int(0);
struct core_string_String core_string_tmpid40 = core_string_s;
return core_string_tmpid40;
}
char* core_string_to_csGG_core_string_String(struct core_string_String core_string_s) {
char* core_string_tmpid55 = ((char*)(core_string_s.p));
return core_string_tmpid55;
}
int64_t core_string_lengthGG_core_string_String(struct core_string_String core_string_s) {
int64_t core_string_tmpid42 = core_string_s.len;
return core_string_tmpid42;
}
char* core_pointer_reallocGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(char* core_pointer_ptr, int64_t core_pointer_len) {
char* core_pointer_tmpid43 = ((char*)(realloc(((void*)(core_pointer_ptr)), (sizeof(char) * core_pointer_len))));
return core_pointer_tmpid43;
}
void core_string_pushGG_core_string_String_core_prelude_CString(struct core_string_String* core_string_dest, char* core_string_src) {
int64_t core_string_l = (core_string_lengthGG_core_string_String(*core_string_dest) + strlen(core_string_src));
core_string_dest->p = core_pointer_reallocGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_string_dest->p, (core_string_l + 1));
core_pointer_memcpyGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_string_dest->p, core_string_lengthGG_core_string_String(*core_string_dest)), ((char*)(core_string_src)), strlen(core_string_src));
(*(core_pointer_plusexclGcore_prelude_CharG_core_pointer_Ptr_core_prelude_Char_core_prelude_Int(core_string_dest->p, core_string_l))) = core_string_charGG_core_prelude_Int(0);
core_string_dest->len = core_string_l;
}
char core_string_cr;
char core_string_lf;
bool core_definitions_is_windows;
bool core_definitions_is_compiletime;
bool core_definitions_is_runtime;
bool core_definitions_is_jscodegen;
bool core_definitions_is_release_fnGG_() {
bool __floritmp1 = core_prelude_true;
bool __floritmp0 = __floritmp1;
bool core_definitions_tmpid65 = __floritmp0;
return core_definitions_tmpid65;
}
bool core_definitions_is_debug_fnGG_() {
bool __floritmp3 = core_prelude_true;
bool __floritmp2 = __floritmp3;
bool core_definitions_tmpid66 = !(__floritmp2);
return core_definitions_tmpid66;
}
bool core_definitions_is_release;
bool core_definitions_is_debug;
#define procon_INF 999999999
#define procon_MOD 1000000007
int64_t procon_read_intGG_() {
int64_t procon_ret;
scanf("%" PRId64, &procon_ret);
int64_t procon_tmpid80 = procon_ret;
return procon_tmpid80;
}
#define d_MAX_N 41
#define d_MAX_AB 11
#define d_MAX_C 4001
int64_t d_n;
int64_t d_ma;
int64_t d_mb;
struct Array40_d_Yakuhin d_yakuhins;
struct Array11_core_array_Array_11_core_prelude_Int d_dp;
int64_t d_minGG_core_prelude_Int_core_prelude_Int(int64_t d_a, int64_t d_b) {
int64_t __floritmp4;
if ((d_a < d_b)) {
__floritmp4 = d_a;
} else {__floritmp4 = d_b;
}int64_t d_tmpid81 = __floritmp4;
return d_tmpid81;
}
void d_dp1GG_() {
{
int64_t d_i = 0;
while ((d_i <= (d_n - 1))) {
struct d_Yakuhin d_yakuhin = ((d_yakuhins).data[d_i]);
if ((((((d_dp).data[d_yakuhin.b])).data[d_yakuhin.a]) > d_yakuhin.c)) {
((&((&d_dp)->data[d_yakuhin.b])))->data[d_yakuhin.a] = d_yakuhin.c;
} else {};
(d_i += 1);
};

}
;
;
;
{
int64_t d_cura = 1;
while ((d_cura <= (d_MAX_AB - 1))) {
{
int64_t d_curb = 1;
while ((d_curb <= (d_MAX_AB - 1))) {
{
int64_t d_preva = 1;
while ((d_preva <= d_cura)) {
int64_t d_prevb = (d_curb - d_preva);
int64_t d_ca = ((((d_dp).data[d_prevb])).data[d_preva]);
int64_t d_cb = ((((d_dp).data[d_preva])).data[d_prevb]);
if ((((((d_dp).data[d_curb])).data[d_cura]) > (d_ca + d_cb))) {
((&((&d_dp)->data[d_curb])))->data[d_cura] = (d_ca + d_cb);
} else {};
(d_preva += 1);
};

}
;
;
;
;
(d_curb += 1);
};

}
;
;
;
;
(d_cura += 1);
};

}
;
;
;
}
void d_mainGG_() {
{
int64_t d_i = 0;
while ((d_i <= (d_n - 1))) {
(&d_yakuhins)->data[d_i] = (struct d_Yakuhin){procon_read_intGG_(), procon_read_intGG_(), procon_read_intGG_()};
;
(d_i += 1);
};

}
;
;
;
{
int64_t d_a = 0;
while ((d_a <= (d_MAX_AB - 1))) {
{
int64_t d_b = 0;
while ((d_b <= (d_MAX_AB - 1))) {
((&((&d_dp)->data[d_b])))->data[d_a] = procon_INF;
;
(d_b += 1);
};

}
;
;
;
;
(d_a += 1);
};

}
;
;
;
d_dp1GG_();
int64_t d_ans = procon_INF;
{
int64_t d_a = 0;
while ((d_a <= (d_MAX_AB - 1))) {
{
int64_t d_b = 0;
while ((d_b <= (d_MAX_AB - 1))) {
if (((d_a * d_mb) == (d_b * d_ma))) {
d_ans = d_minGG_core_prelude_Int_core_prelude_Int(d_ans, ((((d_dp).data[d_b])).data[d_a]));
} else {};
(d_b += 1);
};

}
;
;
;
;
(d_a += 1);
};

}
;
;
;
if ((d_ans == procon_INF)) {
printf("%s", "-1");
;
;
printf("%s", "\n");
;
;
;
} else {printf("%lld", d_ans);
;
;
printf("%s", "\n");
;
;
;
}}

void flori_main() {
core_prelude_true = 1;
core_prelude_false = 0;
core_pointer_null = core_pointer_nullGG_();
core_math_M_PI = M_PI;
core_string_cr = core_string_charGG_core_prelude_Int(13);
core_string_lf = core_string_charGG_core_prelude_Int(10);
core_definitions_is_windows = 
#ifdef _WIN32
#define is_windows true
#elif _WIN64
#define is_windows true
#else
#define is_windows false
#endif
  is_windows;
core_definitions_is_compiletime = 
#ifdef FLORI_COMPILETIME
#define is_compiletime true
#else
#define is_compiletime false
#endif
  is_compiletime;
core_definitions_is_runtime = !(core_definitions_is_compiletime);
core_definitions_is_jscodegen = 
#ifdef FLORI_JSCODEGEN
#define is_jscodegen true
#else
#define is_jscodegen false
#endif
  is_jscodegen;
core_definitions_is_release = core_definitions_is_release_fnGG_();
core_definitions_is_debug = core_definitions_is_debug_fnGG_();
d_n = procon_read_intGG_();
d_ma = procon_read_intGG_();
d_mb = procon_read_intGG_();
d_mainGG_();
}
int main(int argc, char** argv) { flori_main(); }
