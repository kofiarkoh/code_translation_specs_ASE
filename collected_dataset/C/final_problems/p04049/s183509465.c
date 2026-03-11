#include <stdio.h>
#include <stdlib.h>

#define N 2000

typedef struct vertex{
	short int dist[N];
	struct vertex *cnct[N];
	short int length;
}vertex;



int count(vertex* ver,vertex* pre,int k);

int main(){
	int n,k;
	int i;
	int a,b;
	scanf("%d %d",&n,&k);
	vertex *field = malloc(sizeof(vertex));
	for(i = 0;i < n-1;i++){
		scanf("%d %d",&a,&b);
		field[a].dist[b] = 1;
		field[a].cnct[field[a].length++] = &field[b];
		field[b].dist[a] = 1;
		field[b].cnct[field[b].length++] = &field[a];
	}
	b = 0;
	for(i = 1;i <= n;i++){
		if(field[i].length > 1){
			continue;
		}
		a = count(&field[i],NULL,k-1);
		if(a > b){
			b = a;
		}
	}
	printf("%d\n", n - b);
	return 0;
}

int count(vertex* ver,vertex* pre,int k){
	int res = 1;
	int i;
	if(k==0){
		return ver->length;
	}
	for(i = 0;i < ver->length;i++){
		if(ver -> cnct[i] == pre){
			continue;
		}
		res += count(ver -> cnct[i],ver,k-1);
	}
	return res;
}

