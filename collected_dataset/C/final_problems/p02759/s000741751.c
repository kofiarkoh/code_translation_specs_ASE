#include <stdio.h>

//#include <iostream>
//#include <vector>
//#include <string.h>

#pragma warning (push)
#pragma warning (disable : 4996)
#define mod 1000000007;

//using namespace std;

int main()
{
    int N;
    
    scanf("%d", &N);

    char ret[3];
    if (N % 2 == 1) {
        sprintf(ret, "%d", (int)(N / 2 + 1));
    }
    else {
        sprintf(ret, "%d", (int)(N / 2));
    }

    printf(ret);
}

#pragma warning (pop)