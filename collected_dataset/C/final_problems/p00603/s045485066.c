#include <stdio.h>

int riffle_shuffle(int *deckA, int *deckB, int *deckC, int *operation, int n, int r)
{
  int i, ai, bi, ci;
  int oi = 0;

  while(1) {
    ai = bi = ci = 0;
    do {
      for (i = 0; i < operation[oi]; i++) {
        if (deckA[ai] == -1)
          break;
        if (ci < n)
          deckC[ci++] = deckA[ai++];
      }
      for (i = 0; i < operation[oi]; i++) {
        if (deckB[bi] == -1)
          break;      
        if (ci < n)
          deckC[ci++] = deckB[bi++];
      }
    } while (deckA[ai] != -1 || deckB[bi] != -1);

    for (i = 0; i < n/2; i++) {
      deckB[i] = deckC[i];
      deckA[i] = deckC[i + n/2];
    }

    if (n%2 == 1)
      deckA[n/2] = deckC[n-1];

    if (oi > r || operation[++oi] == -1)
      return deckC[--ci];
  }
}


int main(int argc, char const *argv[])
{
  int deckA[50];
  int deckB[50];
  int deckC[50];
  int operation[50];
  int n, r, i, j, last_card;

  while (EOF != scanf("%d %d", &n, &r)) {

    for (i = 0; i < 50; i++)
      deckA[i] = deckB[i] = deckC[i] = operation[i] = -1;
    
    for (i = 0; i < r; i++) 
      scanf("%d", &operation[i]);

    for (i = 0; i < n/2; i++) {
      deckB[i] = i;
      deckA[i] = i + n/2;
    }

    if (n % 2 == 1)
      deckA[n/2] = n-1;

    last_card = riffle_shuffle(deckA, deckB, deckC, operation, n, r);

    printf("%d\n", last_card);

  }
  return 0;
}