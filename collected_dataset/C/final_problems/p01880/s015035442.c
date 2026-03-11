#include <stdio.h>
#include <string.h>
int main(void){
	int n,i,j,k,a[1000],t,f=0,max=-1;
	char tmp[100];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	for(i=0;i<n;i++){
		for(j=i+1;j<n;j++){
			f=1;
			t=a[i]*a[j];
			sprintf(tmp,"%d",t);
			for(k=1;tmp[k];k++){
				if(tmp[k]-tmp[k-1]!=1){
					f=0;
					break;
				}
			}
			if(f&&max<t){
				max=t;
			}
		}
	}
	printf("%d\n",max);
	return 0;
}