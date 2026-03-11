#include <stdio.h>

int main() {
   int n, m; scanf("%d%d", &n, &m);
   int rs = (100 * (18 * m + n)) << m;
   printf("%d\n", rs);
}