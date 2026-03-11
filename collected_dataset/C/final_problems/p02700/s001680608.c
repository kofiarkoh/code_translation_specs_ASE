#include<stdio.h>
int main(void)
{
	int A=0,B=0,C=0,D=0;
  
  	scanf("%d",&A);
  	scanf("%d",&B);
  	scanf("%d",&C);
  	scanf("%d",&D);
  	
  	while(C<=0){
    	C=C-B;
    	if(C>0){
    		A=A-D;
    	}
    }
  	
  	if(A>C){
    	puts("Yes");
    }else{
    	puts("No");
    }
  
  return 0;
  
}
