#include<stdio.h>
#include<stdlib.h>

int comp(const void *p1,const void *p2);

int main(void){
	int k,t,i,*cakes;
	int first,second;
	int tmp,next;

	scanf("%d %d",&k,&t);
	cakes=(int *)calloc(t,sizeof(int));
	for(i=0;i<t;i++){
		scanf("%d",&cakes[i]);
	}
	qsort(cakes,t,sizeof(int),comp);
	
	first=0;
	second=1;
	next=2;
	
	//printf("%d\n",cakes[0]);
	while(first<t && second<t){
		tmp=cakes[first]-cakes[second];
		if(tmp>0){
			cakes[first]-=cakes[second];
			cakes[second]=0;
			second=next++;
		}else if(tmp==0){
			cakes[first]=0;
			for(;cakes[next]==cakes[second];next++){
				cakes[next]=0;
			}
			cakes[second]=0;
			first=next++;
			second=next++;
		}else{
			cakes[second]-=cakes[first];
			cakes[first]=0;
			first=next++;
		}
		//printf("next::%d\n",next);
	}
	if(first==t && second<t){
		printf("%d\n",cakes[second]-1);
	}else if(second==t && first<t){
		printf("%d\n",cakes[first]-1);
	}else{
		printf("%d\n",0);
	}
	free(cakes);
	return 0;
}

int comp(const void *p1,const void *p2){
	int n1,n2;
	n1=*((const int *)p1);
	n2=*((const int *)p2);
	return n2-n1;
}