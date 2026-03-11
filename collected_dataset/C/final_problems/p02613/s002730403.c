#include<stdio.h>
#include<string.h>
 
int main(){
	char a[20] = "AC";
	char b[20] = "WA";
	char c[20] = "TLE";
	char d[20] = "RE";
	int tc;
	int ac = 0;
	int wa = 0;
	int tle = 0;
	int re = 0;
	char s[100];
	scanf("%d", &tc);
	for(int i=0; i<tc; i++){
		scanf("%s", s);
		if(strcmp(s,a) == 0){
			ac++;
		}
		else if(strcmp(s,b) == 0){
			wa++;
		}
		else if(strcmp(s,c) == 0){
			tle++;
		}
		else if(strcmp(s,d) == 0){
			re++;
		}
	}	
		printf("AC x %d\n", ac);
		printf("WA x %d\n", wa);
		printf("TLE x %d\n", tle);
		printf("RE x %d\n", re);
	return 0;
}