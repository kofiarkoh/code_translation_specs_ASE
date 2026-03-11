#include <stdio.h>

int main(void){
	double w;
	char *s;
	
	while( scanf("%lf", &w) !=EOF ) {
		if(w<=48) s="light fly";
		else if(w<=51) s="fly";
		else if(w<=54) s="bantam";
		else if(w<=57) s="feather";
		else if(w<=60) s="light";
		else if(w<=64) s="light welter";
		else if(w<=69) s="welter";
		else if(w<=75) s="light middle";
		else if(w<=81) s="middle";
		else if(w<=91) s="light heavy";
		else s="heavy";
	
		printf("%s\n", s);
	}
	
	return 0;
}
	
	