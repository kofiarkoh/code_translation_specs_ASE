#include <stdio.h>
#include <math.h>

int main(){
	struct point{
		double x,y;
	}P1,P2,Q,R,O;
	double m;

	while(scanf("%lf,%lf,%lf,%lf,%lf,%lf",&P1.x,&P1.y,&P2.x,&P2.y,&Q.x,&Q.y) != EOF){
		if(P1.x != P2.x && P1.y != P2.y){
			m = (P2.y-P1.y)/(P2.x-P1.x);
			O.x = (m/(m*m+1.0))*(m*P1.x+Q.x/m+Q.y-P1.y);
			O.y = m*(O.x-P1.x)+P1.y;
			R.x = 2*O.x-Q.x;
			R.y = 2*O.y-Q.y;
		}else if(P1.x != P2.x && P1.y == P2.y){
			R.x = Q.x;
			R.y = 2*P1.y-Q.y;
		}else if(P1.x == P2.x && P1.y != P2.y){
			R.x = 2*P1.x-Q.x;
			R.y = Q.y;
		}else{
			R.x = 2*P1.x-Q.x;
			R.y = 2*P1.y-Q.y;
		}
		printf("%lf %lf\n",R.x,R.y);
	}

	return 0;
}