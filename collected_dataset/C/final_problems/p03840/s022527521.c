#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
 
#define sl(x) scanf("%ld", &x)
#define ss(x) scanf("%s", x)
#define pl(x) printf("%ld\n", x)
#define ps(x) printf("%s\n", x)
 
#define INF 100000000000
#define MOD 1000000007
 
#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))
 
int main() {
	long N, K;
	long ai, ao, at, aj, al, as, az;
	long i, j;
	long tmp;
	long ans = 0;
	long min_i, max_i;
	
	sl(ai); sl(ao); sl(at); sl(aj); sl(al); sl(as); sl(az);
	
	tmp = 0;
	if(ai % 2) tmp++;
	if(aj % 2) tmp++;
	if(al % 2) tmp++;
	
	if(tmp == 1 || ai == 0 || aj == 0 || al == 0) {
		ans += (ai % 2 ? ai - 1 : ai);
		ans += ao;
		ans += (aj % 2 ? aj - 1 : aj);
		ans += (al % 2 ? al - 1 : al);
	} else if(tmp == 2) {
		ans += 3;
		ai--; aj--; al--;
		ans += (ai % 2 ? ai - 1 : ai);
		ans += ao;
		ans += (aj % 2 ? aj - 1 : aj);
		ans += (al % 2 ? al - 1 : al);
	} else {
		ans += MIN(ai, MIN(aj, al)) * 3;
		ai -= (ans / 3);
		aj -= (ans / 3);
		al -= (ans / 3);
		ans += (ai % 2 ? ai - 1 : ai);
		ans += ao;
		ans += (aj % 2 ? aj - 1 : aj);
		ans += (al % 2 ? al - 1 : al);
	}
	
	pl(ans);
	
	return 0;
}