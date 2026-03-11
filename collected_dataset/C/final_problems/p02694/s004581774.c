#include<stdio.h>
#include<math.h>
#include<string.h>

int main()
{
	long long int A;
	scanf("%ld", &A);
	
	long long int B = 100;
	
	long long int r;
	
	for(r=0; B<A; r++){
		B = B*1.01;
	}
	
	printf("%ld", r);
	

		return 0;
	
}