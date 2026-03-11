#include <stdio.h>
#include <stdlib.h>

int max4(int *);
int min4(int *);
int min(int *, int);

int main()
{
  int n, a[200000];
  int s[4], b0, b1, b2, score[199997];
  int i, j, b0c, b2c;

  scanf("%d", &n);

  for (i = 0; i < n; ++i)
  {
    scanf("%d", &(a[i]));
  }

  for (i = 0; i < n - 1; ++i)
  {
    a[i + 1] = a[i] + a[i + 1];
  }

  b0 = 0;
  b1 = 0;
  b2 = 2;
  for (i = 0; i < n - 3; ++i)
  {
    ++b1;
    b0c = b0;
    for (j = 0; j < b1 - b0c - 1; ++j)
    {
      b0 = b0 + 1;
      if (abs(a[b1] - a[b0] * 2) > abs(a[b1] - a[b0 - 1] * 2))
      {
        b0 = b0 - 1;
        break;
      }
    }
    s[0] = a[b0];
    s[1] = a[b1] - a[b0];

    b2c = b2;
    for (j = 0; j < n - b2c - 2; ++j)
    {
      b2 = b2 + 1;
      if (abs(a[n - 1] - a[b2] * 2 + a[b1]) > abs(a[n - 1] - a[b2 - 1] * 2 + a[b1]))
      {
        b2 = b2 - 1;
        break;
      }
    }
    s[2] = a[b2] - a[b1];
    s[3] = a[n - 1] - a[b2];

//    printf("b0 b1 b2: %d %d %d\n", b0, b1, b2);
//    printf("    s0 s1 s2 s3: %d %d %d %d\n", s[0], s[1], s[2], s[3]);

    score[i] = max4(s) - min4(s);
  }

  printf("%d\n", min(score, n - 3));

  return 0;
}

int max4(int *a)
{
  int ans = a[0], i;
  for (i = 0; i < 3; ++i)
  {
    if (a[i + 1] > ans)
    {
      ans = a[i + 1];
    }
  }
  return ans;
}

int min4(int *a)
{
  int ans = a[0], i;
  for (i = 0; i < 3; ++i)
  {
    if (a[i + 1] < ans)
    {
      ans = a[i + 1];
    }
  }
  return ans;
}

int min(int *a, int size)
{
  int ans = a[0], i;
  for (i = 0; i < size - 1; ++i)
  {
    if (a[i + 1] < ans)
    {
      ans = a[i + 1];
    }
  }
  return ans;
}
