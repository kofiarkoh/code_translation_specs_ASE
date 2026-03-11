#include <stdio.h>
#include <string.h>
#include <stdlib.h>
struct moonlight{
	double eff;
	char l[21];
};
int main(void){
	struct moonlight data[50],tmp;
	int n,p,a,b,c,d,e,f,s,m,i,j;
	double time,income;
	while(1){
		scanf("%d",&n);
		if(!n){
			break;
		}
		for(i=0;i<n;i++){
			scanf("%s %d %d %d %d %d %d %d %d %d",data[i].l,&p,&a,&b,&c,&d,&e,&f,&s,&m);
			time=a+b+c+(d+e)*m;
			income=s*f*m-p;
			data[i].eff=income/time;
		}
		for(i=0;i<n-1;i++){
			for(j=n-1;j>i;j--){
				if(data[j-1].eff<data[j].eff||data[j-1].eff==data[j].eff&&strcmp(data[j-1].l,data[j].l)>0){
					tmp=data[j-1];
					data[j-1]=data[j];
					data[j]=tmp;
				}
			}
		}
		for(i=0;i<n;i++){
			printf("%s\n",data[i].l);
		}
		puts("#");
	}
	return EXIT_SUCCESS;
}