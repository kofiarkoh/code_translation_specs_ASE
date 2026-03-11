#include <stdio.h>
#include <math.h>

int main()
{
int k, snk=0, pw=0;
long num=0;

scanf("%d", &k);

  int i;
while(snk<k) {
	for(i=1; i<10 ;i++) {
		num += powl(10,pw);
		printf("%lld", num);
		snk++;
     	if(snk<k) printf("\n");
        else i=10;
	}
    pw++;
    i=1;
}
return 0;
}
