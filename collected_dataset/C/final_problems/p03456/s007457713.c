#include <stdio.h>
int main() {
int a, b, i, x;
scanf("%d", &a);
scanf("%d", &b);
for (i = 10;i < 101;i = i * 10) {
if (b / i <= 0) {
x = i;
break;
} }
a = a * x;
for (i = 0;1000000;i++) {
if (a == i * i) {
printf("Yes\n");
break;
} if (i == 1000000) {
printf("No\n");
break;
} }
return 0;
}  