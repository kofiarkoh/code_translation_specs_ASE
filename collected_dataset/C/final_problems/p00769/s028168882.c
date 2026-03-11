#include <stdio.h>
#include <stdlib.h>

int int_sort( const void * a , const void * b ) {
  /* 引数はvoid*型と規定されているのでint型にcastする */
  if( *( int * )a < *( int * )b ) {
    return 1;
  }
  else
  if( *( int * )a == *( int * )b ) {
    return 0;
  }
  return -1;
}
int main(){
	char c[100001];
	int i,j,k,l;
	int count,counter;
	int a[1000];
	//int b[1000];
	int max,min;
	
	int x,y;
	
	
	
	
	
	scanf("%d\n",&count);
	for(counter=0;counter<count;counter++){
		fgets(c,10000,stdin);
		//printf("%d\n%s\n",count,c);
		for(y=0;y<1000;y++){
			a[y]=0;
		}
		i=0;
		j=0;
		while(c[i]){
			switch(c[i]){
				case ']':j++;
				case '[':break;
				default:a[j]*=10;a[j]+=c[i]-'0';break;
			}
			i++;
		}
		a[j]=-1;
		max=j;
		
		k=0;
		i=0;
		while(a[k]!=-1){
			/*for(l=0;l<1000;l++){
				b[l]=0;
			}*/
			while(a[i])i++;
			j=(i+1)/2;
			
			k++;
		}
		/*
		for(i=0;i<100;i++){
			printf("a[%d]=%d\n",i,a[i]);
		}
		printf("%d\n",max);
		*/
		i=0;
		j=0;
		k=0;
		l=0;
		while(a[i]!=-1){
			min=i;
			while(a[i]!=0&&a[i]!=-1)i++;
			max=i;
			//printf("%d",max-min);
			qsort((void *)(a+min),max-min,sizeof(int),int_sort);
			a[j]=0;
			for(k=max;k>=(max+min)/2;k--){
				a[j]+=(a[k]+1)/2;
			}
			j++;
			i++;
		}
		a[j]=-1;
		i=0;
		j=0;
		k=0;
		l=0;
		while(a[1]!=-1){
			while(a[i]!=-1){
				min=i;
				while(a[i]!=0&&a[i]!=-1)i++;
				max=i;
				//printf("%d",max-min);
				qsort((void *)(a+min),max-min,sizeof(int),int_sort);
				a[j]=0;
				for(k=max;k>=(max+min)/2;k--){
					a[j]+=a[k];
				}
				j++;
				i++;
			}
			a[j]=-1;
			i=0;
			j=0;
			
		}
		printf("%d\n",a[0]);
		/*
		for(i=0;i<100;i++){
			printf("a[%d]=%d\n",i,a[i]);
		}*/
	}
	return 0;
}