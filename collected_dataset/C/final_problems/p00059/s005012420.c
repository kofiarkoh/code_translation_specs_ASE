#include<stdio.h>
#include<math.h>
#include <stdlib.h>
#include<string.h>
#include<limits.h>

int main(){
	double x[4],y[4];
	int i;
	
	while(scanf("%lf%lf%lf%lf%lf%lf%lf%lf",&x[0],&y[0],&x[1],&y[1],&x[2],&y[2],&x[3],&y[3])!=EOF){
	
		if(x[0]<=x[2] && x[1]>=x[2]){
			if((y[0]<=y[2] && y[1]>=y[2]) || (y[0]<=y[3] && y[1]>=y[3]))printf("YES\n");
			else printf("NO\n");
		}
		else if(x[0]<=x[3] && x[1]>=x[3]){
			if((y[0]<=y[2] && y[1]>=y[2]) || (y[0]<=y[3] && y[1]>=y[3]))printf("YES\n");
			else printf("NO\n");
		}
		else printf("NO\n");
	}
return 0;
}