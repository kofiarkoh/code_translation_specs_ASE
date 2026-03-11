#include <stdio.h>

long Dif3(long a, long b, long c) {
  long min = a;
  long max = a;
  if (b < min) {
    min = b;
  }
  if (c < min) {
    min = c;
  }
  if (b > max) {
    max = b;
  }
  if (c > max) {
    max = c;
  }
  long ans = max-min;
  return ans;
}

int main(void) {

  long h,w;
  scanf("%ld %ld", &h, &w);
  long min = h*w;
  long a,b,c;
  long other,short_side,dif;
  for (long i = 1; i*2 <= w; i++) {
    a = i*h;
    other = w-i;
    if (other%2 == 0 || h%2 == 0) {
      b = other*h/2;
      c = b;
    } else {
      short_side = other;
      if (h < short_side) {
        short_side = h;
      }
      b = (other*h+short_side)/2;
      c = other*h-b;
    }
    dif = Dif3(a,b,c);
    if (dif < min) {
      min = dif;
    }
  }
  for (long i = 1; i*2 <= h; i++) {
    a = i*w;
    other = h-i;
    if (other%2 == 0 || w%2 == 0) {
      b = other*w/2;
      c = b;
    } else {
      short_side = other;
      if (w < short_side) {
        short_side = w;
      }
      b = (other*w+short_side)/2;
      c = other*w-b;
    }
    dif = Dif3(a,b,c);
    if (dif < min) {
      min = dif;
    }
  }
  printf("%ld\n", min);

  return 0;
}