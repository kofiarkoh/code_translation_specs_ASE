#include <stdio.h>
#include <math.h>

int main(void)
{
    int N;
    scanf("%d", &N);
    for (int i = N; i > 0; i--) {
        double value = sqrt((double)i);
        if (ceil(value) - floor(value) == 0) {
            printf("%d\n", i);
            break;
        }
    }

    return 0;
}
