#include <stdio.h>
#include <math.h>
#define X 0
#define Y 1

		/*Op`ÌÊÏÌl*/
double calTriArea(double A[],double B[],double C[])
{
	double a,b,c,
		   s;
	
	a = sqrt( pow(B[X]-C[X],2) + pow(B[Y]-C[Y],2) );
	b = sqrt( pow(C[X]-A[X],2) + pow(C[Y]-A[Y],2) );
	c = sqrt( pow(A[X]-B[X],2) + pow(A[Y]-B[Y],2) );
	
	s = (a+b+c)/2;
	return sqrt( s*(s-a)*(s-b)*(s-c) );
}

int main()
{
	double A[2],B[2],C[2],D[2],
		   S[2];
	int flag,
		i;
	
	while(scanf("%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf"
				,&A[X],&A[Y],&B[X],&B[Y],&C[X],&C[Y],&D[X],&D[Y]) != EOF)
	{
		flag = 0;		/*ÅÍfalthÅZbg*/
		S[0] = 0;
		S[1] = 0;
		
		S[0] += calTriArea(A,B,C) + calTriArea(A,C,D);
		S[1] += calTriArea(A,B,D) + calTriArea(B,C,D);
		
		/*ë·ÅÍËçêéÆ¢âÈÌÅALøº3®ç¢Å*/
		if( (S[0] - 0.0005) <= S[1] && S[1] <= (S[0] + 0.0005) ) flag = 1;
		
		/*	fobOp
		printf("S[0] = %lf  S[1] = %lf\n",S[0],S[1]);
		printf("flag = %d\n",flag);
		printf("%d %d\n\n",(S[0] - 0.0005) <= S[1],S[1] <= (S[0] + 0.0005));
		*/
		
		
		if(flag){
			printf("YES\n");
		} else {
			printf("NO\n");
		}
	}
	
	return 0;
}