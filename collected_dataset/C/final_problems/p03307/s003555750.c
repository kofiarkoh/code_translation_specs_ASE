#include <stdio.h>
int main() {
long long a, b, i;
scanf("%lld", &a);
for (i = 1;;i++) {
if (i % 2 == 0 && i % a == 0) {
b = i;
break;
} }
printf("%lld\n", b);
return 0;
}