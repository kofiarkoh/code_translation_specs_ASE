#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <limits.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define	abs(a) ((a)<0?-(a):(a))



int main(void){
	int n;
	char *s,*t;
	char *sh,*th;
	scanf("%d",&n);
	s = calloc(sizeof(char),n+1);
	t = calloc(sizeof(char),n+1);
	scanf("%s",s);scanf("%s",t);
	sh = s;th = t;
	while(*s){
		if(*s == *th) th++;
		else th = t;
		s++;
	}
	printf("%d\n", 2*n-(th-t));
	return 0;
}