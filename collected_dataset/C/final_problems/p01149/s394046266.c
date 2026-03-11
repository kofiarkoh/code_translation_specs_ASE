#include <stdio.h>

int card(char c){
	if(c=='A'){
		return 11;
	}
	else if((c<'2')||(c>'9')){
		return 10;
	}
	else{
		return (int)c-48;
	}
}

void doit(){
	char c[10];
	char s[17];
	int cn;
	int i;
	int sum=0;
	int f=0;
	scanf("%c",&c[0]);
	scanf("%c %c\n",&c[0],&c[1]);
	for(i=2;i<9;i++){
		scanf("%c ",&c[i]);
	}
	scanf("%c",&c[9]);
	i=0;
	while((i<10)&&(sum<=16)){
		cn=card(c[i]);
		if(cn==11){
			f++;
		}
		sum+=cn;
		if((sum>21)&&(f==1)){
			sum-=10;
		}
		i++;
	}
	if(sum>21){
		printf("bust\n");
	}
	else if((sum==21)&&(i==2)){
		printf("blackjack\n");
	}
	else{
		printf("%d\n",sum);
	}
}

int main(void) {
	int n;
	scanf("%d",&n);
	while(n>0){
		doit();
		n--;
	}
	return 0;
}