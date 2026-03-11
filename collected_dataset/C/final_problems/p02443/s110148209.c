#include<stdio.h>
#include<stdlib.h>


int main(int argc, char *argv[]){
	int i,j;
	int n,b,e,q;
	int *a;
	int tmp;

	scanf("%d",&n);
	a=calloc(n,sizeof(int));

	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}

	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d%d",&b,&e);
		for(j=0;j<=(e-b)/2;j++){
			tmp=a[b+j];
			a[b+j]=a[e-1-j];
			a[e-1-j]=tmp;
		}
	}

	for(i=0;i<n;i++){
		printf("%d ",a[i]);
	}

	exit(0);
}
