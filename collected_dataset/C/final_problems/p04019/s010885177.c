#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

#define _CRT_SECURE_NO_WARNINGS
#define TLong long long
#define TBMod 1000000007

int main(int argc, char const *argv[])
{
	int cnt[4] = {0,0,0,0};	// N,S,E,W
	char path[1007];
	scanf("%s",path);
	for (int i = 0; i < strlen(path); ++i)
	{
		switch(path[i]){
			case 'N':
				++cnt[0];
				break;
			case 'S':
				++cnt[1];
				break;
			case 'E':
				++cnt[2];
				break;
			case 'W':
				++cnt[3];
		}
	}
	if(cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 && cnt[3] > 0)	puts("Yes");
	else if(cnt[0] > 0 && cnt[1] > 0 && cnt[2] == 0 && cnt[3] == 0)	puts("Yes");
	else if(cnt[0] == 0 && cnt[1] == 0 && cnt[2] > 0 && cnt[3] > 0)	puts("Yes");
	else puts("No");
	return 0;
}
