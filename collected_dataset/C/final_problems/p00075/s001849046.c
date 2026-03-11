#include<stdio.h>
int main(){
	double BMI[10000],wg,hi;
	int count,i=0,num[10000];
	
	while(scanf("%d,%lf,%lf",&num[i],&wg,&hi)!=EOF){
	BMI[i]=wg/(hi*hi);
	++i;
	}
	count=i;

	for(i=0;i<count;i++){
		if(BMI[i]>=25)printf("%d\n",num[i]);
	}
return 0;
}