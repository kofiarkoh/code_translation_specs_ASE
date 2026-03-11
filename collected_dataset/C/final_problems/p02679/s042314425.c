#include <stdio.h>
#include <stdlib.h>

typedef struct {
	long a;
  long b;
} S_SARDINE;

long GCD(long a, long b){
  long dividend, divisor;
  if (a >= b) {
    dividend = a;
    divisor = b;
  } else {
    dividend = b;
    divisor = a;
  }
  long remainder;
  while (1) {
    if (divisor == 1) {
      return 1;
    }
    remainder = dividend % divisor;
    if (remainder == 0) {
      break;
    } else {
      dividend = divisor;
      divisor = remainder;
    }
  }
  return divisor;
}

int cmpVal(const void * n1, const void * n2)
{
	if (((S_SARDINE *)n1)->a > ((S_SARDINE *)n2)->a)
  {
    return -1;
  }
  else if (((S_SARDINE *)n1)->a < ((S_SARDINE *)n2)->a)
  {
    return 1;
  }
	else
	{
    if (((S_SARDINE *)n1)->b > ((S_SARDINE *)n2)->b)
  	{
  		return -1;
  	}
  	else if (((S_SARDINE *)n1)->b < ((S_SARDINE *)n2)->b)
  	{
  		return 1;
  	}
    else
    {
      return 0;
    }
	}
}

long modpow(long a, long n, long mod){
	if (n == 0) {
		return 1;
	}
	long answer;
	if (n%2 == 0) {
		answer = modpow(a, n/2, mod);
		answer *= answer;
	} else {
		answer = modpow(a, n-1, mod);
		answer *= a;
	}
	answer %= mod;
	return answer;
}

int main(void) {

  long n;
  scanf("%ld", &n);
  S_SARDINE sardines[n];
  long zero_zero = 0;
  for (long i = 0; i < n; i++) {
    scanf("%ld %ld", &sardines[i].a, &sardines[i].b);
    if (sardines[i].a == 0 && sardines[i].b == 0) {
      zero_zero++;
    }
  }
  long mod = 1000000007;
  long div_a,div_b;
  for (long i = 0; i < n; i++) {
    if (sardines[i].a == 0 && sardines[i].b == 0) {
      continue;
    } else if (sardines[i].a == 0) {
      sardines[i].a = 0;
      sardines[i].b = 1;
    } else if (sardines[i].b == 0) {
      sardines[i].a = 1;
      sardines[i].b = 0;
    } else {
      div_a = sardines[i].a/GCD(labs(sardines[i].a),labs(sardines[i].b));
      div_b = sardines[i].b/GCD(labs(sardines[i].a),labs(sardines[i].b));
      if (div_b < 0) {
        div_a *= -1;
        div_b *= -1;
      }
      sardines[i].a = div_a;
      sardines[i].b = div_b;
    }
  }
  S_SARDINE sardines_a[n],sardines_b[n];
  for (long i = 0; i < n; i++) {
    sardines_a[i].a = sardines[i].a;
    sardines_a[i].b = sardines[i].b;
    if (sardines[i].a > 0) {
      sardines_b[i].a = -sardines[i].b;
      sardines_b[i].b = sardines[i].a;
    } else if (sardines[i].a == 0) {
      sardines_b[i].a = sardines[i].b;
      sardines_b[i].b = sardines[i].a;
    } else {
      sardines_b[i].a = sardines[i].b;
      sardines_b[i].b = -sardines[i].a;
    }
  }
  qsort(sardines_a, n, sizeof(S_SARDINE), cmpVal);
  qsort(sardines_b, n, sizeof(S_SARDINE), cmpVal);

  long start = 0;
  long end;
  long answer = 1;
  long multiple;
  long bad;
  long count_a = 1,count_b;
  long count_a_sum = 0;
  long count_b_sum = 0;
  for (long i = 0; i < n; i++) {
    if (sardines_a[i].a == 0 && sardines_a[i].b == 0) {
      count_a_sum++;
      continue;
    }
    if (sardines_a[i].a < 0) {
      multiple = modpow(2,n-count_a_sum-count_b_sum,mod);
      answer *= multiple;
      answer %= mod;
      break;
    }
    if (i != n-1 && sardines_a[i].a == sardines_a[i+1].a && sardines_a[i].b == sardines_a[i+1].b) {
      count_a++;
    } else {
      while (start < n-1) {
        if (sardines_b[start].a > sardines_a[i].a || (sardines_b[start].a == sardines_a[i].a && sardines_b[start].b > sardines_a[i].b)) {
          start++;
        } else {
          break;
        }
      }
      count_b = 0;
      for (long j = start; j < n; j++) {
        if (sardines_a[i].a == sardines_b[j].a && sardines_a[i].b == sardines_b[j].b) {
          count_b++;
        } else {
          end = j-1;
          break;
        }
      }
      multiple = modpow(2,count_a+count_b,mod);
      bad = modpow(2,count_a,mod)-1;
      bad *= modpow(2,count_b,mod)-1;
      bad %= mod;
      multiple -= bad;
      multiple %= mod;
      answer *= multiple;
      answer %= mod;
      count_a_sum += count_a;
      count_b_sum += count_b;
      count_a = 1;
      start = end+1;
    }
  }
  answer--;
  answer += zero_zero;
  printf("%ld\n", answer);

  return 0;
}
