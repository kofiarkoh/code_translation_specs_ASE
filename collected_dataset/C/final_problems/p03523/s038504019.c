#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define i_cin(X) scanf("%d", &X)
#define i_cin2(X,Y) scanf("%d %d",&X,&Y)
#define i_cin3(X,Y,Z) scanf("%d %d %d",&X,&Y,&Z)
#define l_cin(X) scanf("%ld",&X)
#define l_cin2(X,Y) scanf("%ld %ld",&X,&Y)
#define s_cin(X) scanf("%s",&X)

#define  i_cout(X) printf("%d\n",X)
#define  f_cout(X) printf("%g\n",X)
#define  i_cout2(X,Y) printf("%d %d\n",X,Y)
#define  l_cout(X) printf("%ld\n",X)
#define  s_cout(X) printf("%s\n",X)
#define  ll_cout(X) printf("%lld\n",X)

typedef long long int lli;
#define rept(x, s, n) for (int x = s; x < n; x++)


int i_cins(int n, int* A);
int l_cins2(int n, long* A,long* B);
int s_dsort(const void* a, const void* b);

int main(void) {
	int   len,i;
	char  S[51];

	s_cin(S);
	len = (int)strlen(S);
	if (len < 5 || len>9) { s_cout("NO"); return 0; }

	if (!memcmp(S, "AKIHABARA", 9)) { s_cout("YES"); return 0; }
	for(i=0;i<len;i++)
		if (!memcmp(&S[i], "KIH", 3)) break;
	
	if (i == 4) {
		s_cout("NO"); return 0;
	}
	
	i += 3;
	if (S[i] == 'B') i++;
	else {
		if (!memcmp(&S[i], "AB", 2)) i += 2;
		else {
			s_cout("NO"); return 0;
		}
	}
//	i_cout(i);
//	s_cout("R ");
	if (S[i] == 'R') { s_cout("YES"); return 0; }
	if (!memcmp(&S[i], "AR", 2)) { s_cout("YES"); return 0; }
	s_cout("NO");
	return 0;
}

//降順
int s_dsort(const void* a, const void* b) {
	return(*(int*)b - *(int*)a);
}
//
int l_cins2(int n, long* a, long* b) {
	int i;
	rept(i, 0, n) {
		l_cin2(*(a + i), *(b + i));
	}
	return 0;
}
int i_cins(int n, int* a) {
	int i;
	rept(i, 0, n) {
		i_cin(*(a + i));
	}
	return 0;
}


