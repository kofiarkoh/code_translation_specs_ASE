#include <stdio.h>

int c[3][3];
int a[3];

int main(){

	int i,j,k;

	for(i=0;i<3;i++){
		for(j=0;j<3;j++){
		scanf(" %d",&c[i][j]);
		}
	}

	for(i=0;i<=100;i++){
		a[0]=c[0][0]-i;
		a[1]=c[1][0]-i;
		a[2]=c[2][0]-i;
		if(a[0]>=0&&a[1]>=0&&a[2]>=0){
			if(c[0][1]-a[0]==c[1][1]-a[1] && c[0][1]-a[0]==c[2][1]-a[2] &&
				c[0][2]-a[0]==c[1][2]-a[1] && c[0][2]-a[0]==c[2][2]-a[2]
				){
			printf("Yes\n");
		return 0;
				}
	}
	}

	printf("No\n");


return 0;
}