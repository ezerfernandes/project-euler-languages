-- Euler problem 1: https://projecteuler.net/problem=1
local sum = 0
for i = 1, 999 do
    if i % 3 == 0 or i % 5 == 0 then
        sum = sum + i
    end
end
print(sum)