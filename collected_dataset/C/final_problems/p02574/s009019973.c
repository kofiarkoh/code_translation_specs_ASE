// SPDX-License-Identifier: X11
// 2020-08-31
// Coprime (500pt)

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX 1000005

bool *erato(uint_least32_t bound);

int main() {
  uint_least32_t N;
  scanf("%u\n", &N);

  bool *ps = erato(MAX);
  uint_least32_t *Ap = calloc(MAX, sizeof(uint_least32_t));

  bool set = true;
  bool pair = true;

  uint_least8_t s;
  uint_least32_t A, i, j;
  for (i = 0; i < N - 1; i++) {
    scanf("%u", &A);

    if (ps[A]) {
      if (Ap[A])
        pair = false;
      Ap[A]++;

      continue;
    }

    if (!(A & 1)) {
      if (Ap[2])
        pair = false;
      Ap[2]++;

      while (!(A & 1))
        A >>= 1;

      if (ps[A]) {
        if (Ap[A])
          pair = false;
        Ap[A]++;

        continue;
      }
    }

    if (!(A % 3)) {
      if (Ap[3])
        pair = false;
      Ap[3]++;

      while (!(A % 3))
        A /= 3;

      if (ps[A]) {
        if (Ap[A])
          pair = false;
        Ap[A]++;

        continue;
      }
    }

    for (j = 5, s = 2; j * j <= A; j += s, s = 6 - s)
      if (!(A % j)) {
        if (Ap[j])
          pair = false;
        Ap[j]++;

        while (!(A % j))
          A /= j;

        if (ps[A]) {
          if (Ap[A])
            pair = false;
          Ap[A]++;

          continue;
        }
      }
  }

  scanf("%u", &A);

  if (ps[A]) {
    if (Ap[A])
      pair = false;
    Ap[A]++;

    if (Ap[A] == N)
      set = false;
  }

  if (!ps[A] && !(A & 1)) {
    if (Ap[2])
      pair = false;
    Ap[2]++;

    if (Ap[2] == N)
      set = false;

    while (!(A & 1))
      A >>= 1;

    if (ps[A]) {
      if (Ap[A])
        pair = false;
      Ap[A]++;

      if (Ap[A] == N)
        set = false;
    }
  }

  if (!ps[A] && !(A % 3)) {
    if (Ap[3])
      pair = false;
    Ap[3]++;

    if (Ap[3] == N)
      set = false;

    while (!(A % 3))
      A /= 3;

    if (ps[A]) {
      if (Ap[A])
        pair = false;
      Ap[A]++;

      if (Ap[A] == N)
        set = false;
    }
  }

  if(!ps[A])
    for (i = 5, s = 2; i * i <= A; i += s, s = 6 - s)
      if (!(A % j)) {
        if (Ap[j])
          pair = false;
        Ap[j]++;

        if (Ap[j] == N)
          set = false;

        while (!(A % j))
          A /= j;

        if (ps[A]) {
          if (Ap[A])
            pair = false;
          Ap[A]++;

          if (Ap[A] == N)
            set = false;
        }
      }

  if (pair)
    printf("pairwise coprime\n");
  else if (set)
    printf("setwise coprime\n");
  else
    printf("not coprime\n");

  free(ps);
  free(Ap);
}

bool *erato(uint_least32_t bound) {
  if (!bound)
    return NULL;

  bool *array = calloc(bound, sizeof(bool));
  uint_least32_t i,j;
  uint_least8_t s,ss;

  if (bound < 3)
    return array;
  array[2] = true;

  if (bound < 4)
    return array;
  array[3] = true;

  if (bound < 5)
    return array;

  for (i = 5, s = 2; i < bound; i += s, s = 6 - s)
    array[i] = true;

  for (i = 5, s = 2; i < bound; i += s, s = 6 - s)
    if (array[i])
      for (j = i, ss = s; i * j < bound; j += ss, ss = 6 - ss)
        array[i * j] = false;

  return array;
}
