#include <stdio.h>
#define N 10
void sort(int *data,int size);
int main(){
	int a[N],i;
	for(i=0;i<N;i++)scanf("%d",&a[i]);
	sort(a,N);
	for(i=0;i<3;i++)printf("%d\n",a[i]);
	return 0;
}
void sort(int *data,int size){
	int i,j,swap;
	for(i=0;i<size;i++){
		for(j=i+1;j<size;j++){
			if(data[i]<data[j]){
				swap=data[i];
				data[i]=data[j];
				data[j]=swap;
			}
		}
	}
}