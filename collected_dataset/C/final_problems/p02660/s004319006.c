#include <stdio.h>
 
int main() {
    long long int n;
    scanf("%lld", &n);
	if (n == 1) {
		printf("0");
		return 0;
	}
	int count2 = 0;
    int ans=0;
	while (n % 2 == 0) {
		//printf("2 ");
		n /= 2;
		count2++;
	}
    int aa=1;
    while(count2>=1){
        //printf("--%d\n",count2);
        count2-=aa;
        aa++;
        if(count2>=0){ans++;}
    }
    //printf("--%d\n",ans);
    int count;
	for (int i = 3; i <= 1000000; i++) { 
	    count=0;
		while (n % i == 0) {
			//printf("%d ", i);
			n /= i;
			count++;
		}
		aa=1;
		
		while(count>=1){
		//printf("--%d\n",count);
        count-=aa;
        aa++;
        if(count>=0){ans++;}
        //printf("--ans %d\n",ans);
		}
		count=0;
	}
	if (n > 100000) {
		ans++;
	}
    printf("%d", ans);
	return 0;
}