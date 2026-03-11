#include <stdio.h>

int main(){
	int N;
	scanf("%d",&N);
	float a[N];
	float alt[N];
	for (int i = 0; i < N; ++i)
	{
		scanf("%f",&a[i]);
	}
	float total=0;
	float average;
	for (int i = 0; i < N; ++i)
	{
		total += a[i];
	}
	average = total/N;
	for (int i = 0; i < N; ++i)
	{
		if(a[i]<0){
			a[i] = (a[i]-average)*(-1);
		}
		else{
			a[i] -= average;
		}
		alt[i] = a[i];
	}
	for (int i = N; i > 0; --i)
	{
		for (int j = 0; j < i; ++j)
		{
			if(a[j] < a[j-1]){
				float tmp = a[j];
				a[j] = a[j-1];
				a[j-1] = tmp;
			}
		}
	}
	int answer;
	for (int i = 0; i < N; ++i)
	{
		if(alt[i]==a[0]){
			answer = i;
			break;
		}	
	}
	printf("%d",answer);	
	return 0;
}
