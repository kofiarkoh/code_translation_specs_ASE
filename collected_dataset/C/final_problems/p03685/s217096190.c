#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define swap(type,a,b) do{type t=a;a=b;b=t;}while(0);
#define MAX(a,b) (((a)>(b))?(a):(b))
#define MIN(a,b) (((a)<(b))?(a):(b))
#define ll long long
#define INF 100000000
#define FOR(i,a,n) for(i=a;i<n;i++)
void fill(int a[],int b,int c){
	int i;
	FOR(i,0,b) a[i]=c;
	return;
}
typedef struct{
	int a,b;
}Line;
int comp(const void* a,const void* b){
	return ((Line*)a)->a - ((Line*)b)->a;
}
Line line1[100000],line2[100000];
int main(void)
{
	int r,c,n,dx,dy,i;
	int s1=0,s2=0,h=0,w=0;
	int X1,Y1,X2,Y2;
	scanf("%d%d%d",&r,&c,&n);
	FOR(i,0,n){
		scanf("%d%d%d%d",&X1,&Y1,&X2,&Y2);
		if(X1>X2) swap(int,X1,X2);
		if(Y1>Y2) swap(int,Y1,Y2);
		dx=X2-X1;
		dy=Y2-Y1;
		if(dx==r||dy==c){
			if(h||w){
				puts("NO");
				return 0;
			}
			h=w=1;
			line1[s1].a=X1,line1[s1++].b=X2;
			line2[s2].a=Y1,line2[s2++].b=Y2;
		}
		else if(dx==r){
			if(h){
				puts("NO");
				return 0;
			}
			w=1;
			line2[s2].a=Y1,line2[s2++].b=Y2;
		}
		else if(dy==c){
			if(w){
				puts("NO");
				return 0;
			}
			h=1;
			line1[s1].a=X1,line1[s1++].b=X2;
		}
	}
	if(s1==0&&s2==0){
		puts("YES");
		return 0;
	}
	qsort(line1,s1,sizeof(Line),comp);
	FOR(i,0,s1-1){
		if(line1[i].b>line1[i+1].b){
			puts("NO");
			return 0;
		}
	}
	qsort(line2,s2,sizeof(Line),comp);
	FOR(i,0,s2-1){
		if(line2[i].b>line2[i+1].b){
			puts("NO");
			return 0;
		}
	}
	puts("YES");
	return 0;
}
    
    
