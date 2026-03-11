#include<stdio.h>
int main(void) {
	long long int q1, b, c1, c2, q2;
	long long  int aizu_c, normal_c=0,leaveMoney;
	while (1) {
		scanf("%lld", &q1);
		if (q1 == 0)break;
		scanf("%lld %lld %lld %lld", &b, &c1, &c2, &q2);

		aizu_c = q2;
		leaveMoney = b - (aizu_c* c1);
		while (leaveMoney < 0) {
			aizu_c--;
			leaveMoney = b - (aizu_c * c1);
		}
		normal_c = leaveMoney / c2;
		while ( normal_c + aizu_c < q1) {
			aizu_c--;
			leaveMoney = b - (aizu_c * c1);
			normal_c = leaveMoney / c2;
		}
		if (aizu_c > 0)
			printf("%lld %lld\n", aizu_c, normal_c);
		else
			printf("NA\n");
	}
	return 0;
}