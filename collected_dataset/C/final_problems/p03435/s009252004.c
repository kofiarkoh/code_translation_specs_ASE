#include <stdio.h>

int c[3][3]={0};

int main(){
    for (int i=0;i<3;i++){
	for (int j=0;j<3;j++){
	    scanf("%d",&c[i][j]);
	}
    }

    int sub=0;
    for (int i=1;i<3;i++){
       	sub=c[i][0]-c[i-1][0];
	for (int j=1;j<3;j++){
	    if (sub != (c[i][j]-c[i-1][j])){
		printf("No\n");
		return 0;
	    }
	}
    }

    printf("Yes\n");

    return 0;
}