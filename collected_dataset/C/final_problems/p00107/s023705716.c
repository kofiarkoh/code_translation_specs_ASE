#include<stdio.h>
#include<math.h>

int func(int* a,int* b){
	return ((*a)-(*b));
}
int main (){
	int v[3];
	int s;
	int n;
	int i;
	while('A'){
		scanf("%d %d %d",&v[0],&v[1],&v[2]);
		if(v[0]==0){
			break;
		}
		qsort(v,3,sizeof(int),func);
		scanf("%d",&n);
		for(i=0;i<n;i++){
			scanf("%d",&s);
			if((v[0]*v[0]+v[1]*v[1])<(4*s*s)){
				puts("OK");
			}
			else {
				puts("NA");
			}
		}
	}
	return 0;
}