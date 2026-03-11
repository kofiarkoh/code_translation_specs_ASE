#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int32_t i32;
typedef int64_t i64;

static void print_int(i64 n){if(n<0){putchar('-');n=-n;}if(n==0){putchar('0');return;}int s[20],len=0;while(n>0){s[len++]=n%10+'0';n/=10;}while(len>0){putchar(s[--len]);}}
static i64 read_int(void){int prev='\0';int c=getchar();while(!('0'<=c && c<='9')){prev=c;c=getchar();}i64 res=0;while('0'<=c && c<='9'){res=10*res+c-'0';c=getchar();}return prev=='-'?-res:res;}

typedef struct range {
  i32 l, r;
} range;

void add (i32 *bit, i32 x, i32 v) {
  i32 n = bit[0];
  for (i32 i = x; i <= n; i += i & -i) {
    bit[i] += v;
  }
}

i32 find (i32 *bit, i32 x) {
  i32 sum = 0;
  for (i32 i = x; i > 0; i -= i & -i) {
    sum += bit[i];
  }
  return sum;
}

void run (void) {
  i32 n = read_int();
  i32 m = read_int();
  range **p = (range **) calloc (m + 1, sizeof (range *));
  i32 *len = (i32 *) calloc (2 * (m + 1), sizeof (i32));
  i32 *max_len = len + m + 1;
  for (i32 i = 0; i < n; ++i) {
    i32 l = read_int();
    i32 r = read_int() + 1;
    i32 d = r - l;
    if (len[d] == max_len[d]) {
      max_len[d] = 2 * max_len[d] + 1;
      p[d] = realloc (p[d], max_len[d]);
    }
    p[d][len[d]++] = (range) {l, r};
  }
  i32 *bit = (i32 *) calloc (m + 2, sizeof (i32));
  bit[0] = m + 1;
  for (i32 d = 1, c = 0; d <= m; ++d) {
    for (i32 i = 0; i < len[d - 1]; ++i) {
      c++;
      add (bit, p[d - 1][i].l, 1);
      add (bit, p[d - 1][i].r, -1);
    }
    i32 ans = n - c;
    for (i32 j = d; j <= m; j += d) {
      ans += find (bit, j);
    }
    print_int (ans);
    putchar ('\n');
  }
}

int main (void) {
  run();
  return 0;
}
