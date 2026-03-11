#include <stdio.h>
int main(){
char Stat1, Stat2, Stat3;
scanf("%c %c %c",&Stat1, &Stat2, &Stat3);
if((Stat1 == Stat2 && Stat1 == Stat3 && Stat2 == Stat3)||(Stat1 != Stat2 && Stat1!= Stat3 && Stat2 != Stat3)){
	printf("No");
}
else{
	printf("Yes");
}
return 0;
}