#include<stdlib.h>
#include<stdio.h>
#include<math.h>
#define TRUE 0
#define FALSE 1

int main(void) {
	int i;
	int j;
	int max=(int)pow(2.0,15.0);
	int hurui[max];
	
	/////?´???°?±???????/////
	for(i=0;i<max;i++) {
		hurui[i] = TRUE;
	}
	hurui[0] = FALSE;
	hurui[1] = FALSE;
	
	for(i=2;i<max;i++) {
		for(j=2;j<max;j++) {
			if(i*j<max) {
				hurui[i*j] = FALSE;
			}
		}
	}
	/*for(i=0;i<100;i++) {
		if(hurui[i]==TRUE) printf("%d = ?´???°\n",i);
    }*/
	/////////////////////
	
	int input;
	int count;
	
	while(1) {
		scanf("%d",&input);
		if(input==0) break;
		
		count = 0;
		
		for(i=2; i<=input; i++) {
			for(j=2; j<=i; j++) {
				//printf("%d %d\n",i,j);
				if(hurui[i] == FALSE || hurui[j] == FALSE) continue;
				if( (i+j) == input ) {
					count++;
					//printf("%d = %d+%d\n",input,i,j);
				}
			}
		}
		printf("%d\n",count);
	}
	
	exit(0);
}