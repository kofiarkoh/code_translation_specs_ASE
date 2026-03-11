#include<stdio.h>

int main(void)
{
int N, A, B;
scanf("%d %d %d", &N, &A, &B);

if ((B - A - 1)%2 == 1) {
printf("Alice");
} else {
printf("Borys");
}
  return 0;
}