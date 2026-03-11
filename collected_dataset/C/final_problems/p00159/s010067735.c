#include <stdio.h>
#include <math.h>
#include <stdlib.h>

double h,w,bmi,best;
int n,i,num;

int main(){
	while(scanf("%d",&n)*n){
		best=9999999;
		for(;n>0;n--){
			scanf("%d %lf %lf",&i,&h,&w);
			bmi=fabs(22-w*10000/h/h);
			if(bmi<best || (bmi==best && i<num)){
				best=bmi;
				num=i;
			}
		}
		printf("%d\n",num);
	}
	return 0;
}