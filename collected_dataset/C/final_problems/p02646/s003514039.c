#include <stdio.h>
#include <stdlib.h>

int main() {
	long long a, v, b, w, t;
	scanf("%lld%lld%lld%lld%lld", &a, &v, &b, &w, &t);
	if (a < b) {
		if ((v - w) * t >= b - a)printf("YES");
		else printf("NO");
	}
	else {
		if ((v - w) * t >= a- b)printf("YES");
		else printf("NO");
	}
	return  0;
}