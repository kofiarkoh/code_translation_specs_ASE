#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define PI 3.14159265358979323

//int calc1(float a,float b,float c,float d,float e,float f,float *x,float *y);
//void quick(int *,long long,long long);
//int comp(char *a,char *b);

int main(){
	
	int i,j,n,max1,max2,temp,x,flag;

	scanf("%d",&n);
	for(i=0;i<n;i++){
		max1=0;
		max2=0;
		flag=1;
		for(j=0;j<10;j++){
			scanf("%d",&x);
			if(x>max1)
				max1=x;
			else if(x>max2)
				max2=x;
			else{
				flag=0;
			}
		}
		if(flag==1)
			printf("YES\n");
		else
			printf("NO\n");
	}
	return 0;
}