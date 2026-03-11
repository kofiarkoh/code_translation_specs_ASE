#include <stdio.h>

int main()
{
	int N,A, B, i, x, y;
	
	scanf("%d", &N);
	scanf("%d", &A);
	scanf("%d", &B);
	
	char s[N];
	
	scanf("%s", s);
	
	x = 0;
	y = 0;
	
	
	for (i = 0; i < N; i++)
	{
		switch (s[i]) 
		{
			case 'a':
                if(x < A + B)
				{
                    printf("Yes\n");
                    x++;
                }
				else
				{
                    printf("No\n");
                }
                break;
            case 'b':
                if(x < A + B && y < B)
				{
                    printf("Yes\n");
                    x++;
                    y++;
                }
				else
				{
                    printf("No\n");
                }
                break;
            case 'c':
                printf("No\n");
                break;
		}
	
	}
	return 0;
}