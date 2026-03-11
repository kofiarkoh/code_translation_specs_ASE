#include <stdio.h>
int main(){
	int A,B,C;
	scanf("%d %d %d",&A,&B,&C);
  	int Cleft;
	if(A==B){
    	Cleft=C;
    }else if(A-B>C){
		Cleft=0;
	}else{
		Cleft=C-(A-B);
	}
	printf("%d",Cleft);
	return 0;
}