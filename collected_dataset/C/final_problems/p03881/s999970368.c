#include <stdio.h>

int N = 6;
double p[6], q[6];

double max(double a, double b){
	return a >= b ? a : b;
}

double Pr(double x){
	int i;
	double ans = 0;
	for(i = 0; i < N; i++){
		ans += max(p[i] * x, q[i] * (1 - x));
//		printf("ans[%d] = %lf\n", i, ans);
	}
	return ans;
}

int main(){
	int i;
	for(i = 0; i < N; i++){
		scanf("%lf", &p[i]);
		p[i] /= 100;
	}
	for(i = 0; i < N; i++){
		scanf("%lf", &q[i]);
		q[i] /= 100;
	}
/*	for(i = 0; i < N; i++){
		printf("%lf\n", p[i]);
	}
	for(i = 0; i < N; i++){
		printf("%lf\n", q[i]);
	}
*/	double l = 0, h1, h2, r = 1;
//	printf("Pr(%lf) = %lf\n", l, Pr(l));
//	printf("Pr(%lf) = %lf\n", r, Pr(r));
	for(i = 0; i < 100; i++){
		h1 = (l + l + r) / 3;
		h2 = (l + r + r) / 3;
		if(Pr(h1) < Pr(h2)){
			r = h2;
		}
		else{
			l = h1;
		}
	}
//	printf("%.12lf\n", (l + r) / 2);
	printf("%.12lf\n", Pr((l + r) / 2));
	return 0;
}