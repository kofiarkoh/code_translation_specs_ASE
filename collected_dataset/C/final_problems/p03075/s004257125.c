#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int intsort(const void *a, const void *b){
	return *(int*)a - *(int*)b;
}

int main(){
	int a[5],k,i;
	for(i=0;i<5;i++){
		scanf("%d",&a[i]);
	}
	scanf("%d",&k);

	qsort(a,5,sizeof(int),intsort);

	if(k<a[4]-a[0]){
		printf(":(\n");
	}else{
		printf("Yay!\n");
	}

	return 0;
}