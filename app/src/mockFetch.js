export const mockFetch = ({ textResult }) => {
  return jest.fn(() =>
    Promise.resolve({
      text: () => Promise.resolve(textResult)
    })
  );
};
