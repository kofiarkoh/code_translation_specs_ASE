#include <stdio.h>
#include <stdlib.h>
#include <math.h>


int main(void){
	int n,m;
	int **p;
	int max,max2;

	while(1){
		scanf("%d %d",&n,&m);
		if(n==0 && m==0){break;}
		p=(int**)malloc(m*sizeof(int*));
		for(int i=0;i<m;i++){
			p[i]=(int*)malloc(n*sizeof(int));
			for(int j=0;j<n;j++){
				scanf("%d",&p[i][j]);
			}
		}

		max=0;
		for(int i=0;i<m;i++){
			max+=(p[i][0]);
		}

		for(int i=1;i<n;i++){
			max2=0;
			for(int j=0;j<m;j++){
				max2+=(p[j][i]);
			}
			if(max < max2){
				max=max2;
			}
		}
		printf("%d\n",max);
	}

    return 0;
}
