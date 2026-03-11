#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define true 1
#define false 0

int score[123];

typedef struct truck {
	char *p;
	int id;
} truck;

int strcompalpha(char *a,char *b){
	while(*a!='\0'&&*b!='\0'){
		if(*a==*b){
			a++;
			b++;
			continue;
		}
		if(score[*a]<score[*b]){
			return 0;
		}else{
			return 1;
		}
		a++;
		b++;
	}
	if(*a=='\0'){
		return 0;
	}else{
		return 1;
	}
}



int query(truck *s,int k,int n,char *alpha){
	int i;
	int res=1;
	for(i=0;i<26;i++){
		score[alpha[i]] = i;
	}

	for(i=0;i<n;i++){
		res += strcompalpha(s[k].p,s[i].p);
	}
	return res;
}



int main(void){
	int i;
	truck *s;
	char *c;
	int counter;
	int n;
	int q;
	int k;
	char *alpha;
	scanf("%d",&n);
	c = calloc(sizeof(char),400000);
	s = calloc(sizeof(truck),n);
	alpha = calloc(sizeof(char),27);
	for(i=0;i<n;i++){
		scanf("%s",c);
		counter = 0;
		while(*c++)counter++;
		s[i].p = calloc(sizeof(char),counter+1);
		s[i].id = i+1;
		c-=counter+1;
		strcpy(s[i].p,c);
	}
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %s",&k,alpha);
		printf("%d\n", query(s,k-1,n,alpha));
	}
	return 0;
}