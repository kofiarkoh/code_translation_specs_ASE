#include <stdio.h>
#include <stdlib.h>

int mode=1000000007;

int main(){
	int N;
	scanf("%d",&N);
	char S1[N+1];
	char S2[N+1];
	scanf("%s",S1);
	scanf("%s",S2);
	int i=0;
	long int ans=0;
	while(i<N){
		if (ans>mode){
			ans=ans%mode;
		}
		if (i==0){
			if (S1[0]==S2[0]){
				ans+=3;
				i++;
			}
			else{
				ans+=6;
				i+=2;
			}
		}
		else{
			if (S1[i]==S2[i]){
				if (S1[i-1]==S2[i-1]){
					ans=ans*2;
				}
				else{
					ans=ans;
				}
				i++;
			}
			else{
				if (S1[i-1]==S2[i-1]){
					ans=ans*2;
				}
				else{
					ans=ans*3;
				}
				i+=2;
			}
		}
	}
	if (ans>mode){
		ans=ans%mode;
	}
	printf("%ld\n",ans);
	return 0;

}